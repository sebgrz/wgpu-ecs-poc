# WebGPU Bind Groups and Texture Views - Complete Guide

## Table of Contents
1. [Binding Group Strategy](#binding-group-strategy)
2. [Texture Views Explained](#texture-views-explained)
3. [Relationships Between Texture, Views, and Bind Groups](#relationships)
4. [Bind Groups from GPU Perspective](#gpu-perspective)
5. [Dynamic Texture Changes](#dynamic-texture-changes)
6. [Queue Write Texture](#queue-write-texture)

---

## Binding Group Strategy {#binding-group-strategy}

### Single Binding Group + Update Resources (Generally Better)

Create one `GPUBindGroupLayout` with texture bindings, then create multiple `GPUBindGroup` instances with different texture resources.

**Advantages:**
- **Better for frequently changing textures** - Just create new bind groups without recreating the layout
- **Less overhead** - Fewer bind group layouts = fewer cached objects
- **Simpler shader code** - Single shader doesn't need to change
- **Better for dynamic scenarios** - Loading/streaming different textures at runtime

**When to use:** Post-processing effects, material swaps, texture atlases, dynamic asset loading

```javascript
// Single shader
@group(0) @binding(0) var inputTexture: texture_2d<f32>;
@group(0) @binding(1) var texSampler: sampler;
```

### Multiple Binding Groups (For Specific Cases)

Create separate `GPUBindGroupLayout` and `GPUBindGroup` objects for each texture variant.

**Advantages:**
- **Better for static/known textures** - If you know all textures upfront, this can be optimized by drivers
- **Explicit clarity** - Each binding group represents a distinct configuration
- **Potential driver optimization** - Some drivers may better optimize fixed binding layouts

**When to use:** Fixed texture sets that don't change, specialized pipelines for specific texture types

### Practical Recommendation

Use **Option 1** (single layout, multiple bind groups) as your default because:
1. It's more flexible for real-world applications where textures change
2. It reduces memory overhead from duplicate layouts
3. Switching bind groups at draw time is a lightweight operation in WebGPU
4. You can still optimize by batching textures that use the same layout

```javascript
// Single layout
const layout = device.createBindGroupLayout({
  entries: [{
    binding: 0,
    visibility: GPUShaderStage.FRAGMENT,
    texture: { sampleType: 'float' }
  }]
});

// Multiple bind groups with different textures
const bindGroup1 = device.createBindGroup({
  layout, entries: [{ binding: 0, resource: textureView1 }]
});

const bindGroup2 = device.createBindGroup({
  layout, entries: [{ binding: 0, resource: textureView2 }]
});
```

This approach matches WebGPU's design philosophy of separating layout definitions from resource bindings.

---

## Texture Views Explained {#texture-views-explained}

A `GPUTextureView` is an **interface/reference** to a texture that specifies **which part and how to interpret it**. It's needed because a single texture can be viewed in multiple different ways.

### Why You Need TextureViews

#### 1. Specify a Subset (Mip Levels, Array Layers)

```javascript
const texture = device.createTexture({
  size: { width: 256, height: 256, depthOrArrayLayers: 4 },
  format: 'rgba8unorm',
  usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT,
  mipLevelCount: 4
});

// View only mip level 2
const mipView = texture.createView({
  baseMipLevel: 2,
  mipLevelCount: 1
});

// View only layer 1
const layerView = texture.createView({
  baseArrayLayer: 1,
  arrayLayerCount: 1
});
```

#### 2. Change Interpretation (Format, Aspect)

```javascript
// View depth aspect only (for depth textures)
const depthView = depthTexture.createView({
  aspect: 'depth-only'
});

// View stencil aspect only
const stencilView = depthTexture.createView({
  aspect: 'stencil-only'
});
```

#### 3. Change Dimension

```javascript
// Same texture, viewed as 2D-array instead of 3D
const view2DArray = texture3d.createView({
  dimension: '2d-array'
});
```

### Simple Answer

- **Texture** = the actual GPU memory data
- **TextureView** = a "window" into that texture (what part to see, how to interpret it)

You bind **views** (not textures) to bind groups because views let you expose different parts of the same texture to different shaders or different ways to the same shader.

```javascript
// Wrong - can't do this
bindGroup = device.createBindGroup({
  entries: [{ binding: 0, resource: texture }]  // ❌ Error
});

// Right - use a view
bindGroup = device.createBindGroup({
  entries: [{ binding: 0, resource: texture.createView() }]  // ✅ Correct
});
```

---

## Relationships Between Texture, Views, and Bind Groups {#relationships}

### The Hierarchy

One texture can have multiple texture views, and each texture view can be used in multiple bind groups.

```
Texture (GPU memory)
├── TextureView 1 (mip level 0)
│   ├── BindGroup 1
│   ├── BindGroup 2
│   └── BindGroup 3
├── TextureView 2 (mip level 1)
│   ├── BindGroup 4
│   └── BindGroup 5
└── TextureView 3 (array layer 1)
    └── BindGroup 6
```

### Key Points

1. **One texture** can have **multiple views** ✅

```javascript
const view1 = texture.createView({ baseMipLevel: 0 });
const view2 = texture.createView({ baseMipLevel: 1 });
const view3 = texture.createView({ baseArrayLayer: 1 });
```

2. **One texture view** can be used in **multiple bind groups** ✅

```javascript
const view = texture.createView();

const bindGroup1 = device.createBindGroup({
  layout: layout1,
  entries: [{ binding: 0, resource: view }]
});

const bindGroup2 = device.createBindGroup({
  layout: layout2,
  entries: [{ binding: 1, resource: view }]  // Same view, different bind group
});
```

3. **One bind group** can contain **multiple texture views** ✅

```javascript
const bindGroup = device.createBindGroup({
  entries: [
    { binding: 0, resource: view1 },
    { binding: 1, resource: view2 },
    { binding: 2, resource: view3 }
  ]
});
```

### Common Pattern

```javascript
// Single texture with default view
const texture = device.createTexture({...});
const view = texture.createView();  // One default view

// Create multiple bind groups using same view
const bgForMaterial1 = device.createBindGroup({...view...});
const bgForMaterial2 = device.createBindGroup({...view...});
```

**So it's one-to-many at each level:** one texture has many views, and each view can be in many bind groups.

---

## Bind Groups from GPU Perspective {#gpu-perspective}

Yes, bind groups exist **in GPU memory** and are **structures that track which resources are bound**.

### What a Bind Group Contains

A bind group is essentially a **descriptor table/array** that GPU understands:

```
┌─────────────────────────────────────┐
│         Bind Group (GPU Memory)     │
├─────────────────────────────────────┤
│ Binding 0: TextureView pointer      │ → Points to GPU texture memory
│ Binding 1: Sampler settings         │ → Filtering, wrapping settings
│ Binding 2: Buffer GPU address       │ → Offset in GPU buffer memory
│ Binding 3: StorageTexture pointer   │ → Points to GPU texture memory
└─────────────────────────────────────┘
```

### From the Shader's Perspective

When your shader does:

```wgsl
@group(0) @binding(0) var myTexture: texture_2d<f32>;

let color = textureSample(myTexture, sampler, uv);
```

The GPU:
1. Looks up **bind group 0**
2. Looks up **binding 0** in that group
3. Gets the **GPU memory address** of the texture
4. Performs the sample operation

### What Happens at Creation

```javascript
const bindGroup = device.createBindGroup({
  layout,
  entries: [
    { binding: 0, resource: textureView }
  ]
});
```

The GPU driver:
1. **Allocates GPU memory** for the bind group descriptor table
2. **Writes the GPU addresses/pointers** of your resources into that table
3. **Returns a handle** (`GPUBindGroup`) to access it later

### At Render Time

```javascript
passEncoder.setBindGroup(0, bindGroup);
passEncoder.draw(...);
```

The GPU driver:
1. **Sets the bind group pointer** in the GPU's command stream
2. GPU executes shaders with **direct access** to those resources
3. Texture sampling, buffer reads, etc. use the addresses stored in the bind group

### Key Insight

Bind groups are **GPU-side lookup tables**. Instead of telling GPU "use this exact texture at address X", you say "use bind group 0" and the GPU knows to look up all the resource addresses from that bind group structure.

**This is why bind groups are immutable** - once created, the GPU memory addresses are fixed. Changing them would require rewriting GPU memory.

---

## Dynamic Texture Changes {#dynamic-texture-changes}

### Can You Change Texture Resources Dynamically?

No, you **cannot change the texture resource in a bind group after it's created**. Bind groups are immutable once created.

### Option 1: Create a New Bind Group (Recommended)

```javascript
// Create new bind group with different texture
const bindGroup = device.createBindGroup({
  layout, 
  entries: [{ binding: 0, resource: textureView }]
});

// Use it in your render pass
passEncoder.setBindGroup(0, bindGroup);
```

This is the intended approach. Bind groups are lightweight objects, so creating new ones as needed is fine.

### Option 2: Use a Texture Array (Advanced)

If you need to switch between multiple textures frequently without creating new bind groups, use a texture binding array:

```wgsl
@group(0) @binding(0) var textures: binding_array<texture_2d<f32>>;
@group(0) @binding(1) var samplers: binding_array<sampler>;

fn main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
  let textureIndex = u32(someValue);
  let color = textureSample(textures[textureIndex], samplers[0], uv);
  return color;
}
```

**Note:** Texture arrays require the `"texture_binding_array"` feature to be enabled on your device.

### Best Practice

For most cases, simply **create new bind groups as needed**. Modern WebGPU implementations handle this efficiently, and it's the cleanest, most predictable approach.

---

## Queue Write Texture {#queue-write-texture}

### What Does `queue.writeTexture()` Do?

`queue.writeTexture()` **transfers data from CPU memory to GPU memory**.

```javascript
device.queue.writeTexture(
  { texture: gpuTexture },           // GPU texture destination
  cpuData,                            // CPU-side data (ArrayBuffer, TypedArray, etc.)
  { bytesPerRow: 256 },              // Data layout info
  { width: 256, height: 256 }        // Region size
);
```

### The Flow

1. **CPU → GPU Transfer**: Data from `cpuData` (CPU memory) is copied to `gpuTexture` (GPU memory)
2. **Blocking Operation**: The function returns after the transfer is submitted to the GPU queue
3. **Asynchronous Execution**: The actual GPU write happens asynchronously, but the data is guaranteed to be written before any command that uses that texture

### Important Notes

- **Use for small/moderate data**: Good for initialization, dynamic updates, or small texture uploads
- **Not ideal for large streaming**: For large textures or frequent large updates, consider using buffer copies or compute shaders instead
- **GPU memory only**: Once written, the data lives in GPU memory. It won't automatically sync back to CPU
- **Synchronization**: Subsequent commands in the queue that read this texture will see the written data

### Example

```javascript
const imageData = new Uint8Array([...pixel data...]);

device.queue.writeTexture(
  { texture: myGPUTexture },
  imageData,
  { bytesPerRow: width * 4 },  // 4 bytes per pixel (RGBA)
  { width, height, depthOrArrayLayers: 1 }
);

// Now myGPUTexture contains the data in GPU memory
// and can be used in render passes
```

---

## Summary

| Concept | Purpose | Mutable? |
|---------|---------|----------|
| **Texture** | GPU memory storage for image data | Mutable (via writeTexture, copies) |
| **TextureView** | Window into a texture (mip level, array layer, aspect) | Immutable |
| **BindGroup** | GPU-side lookup table of resources | Immutable |
| **BindGroupLayout** | Schema defining bind group structure | Immutable |

**Best Practices:**
- Create one layout, multiple bind groups for different resources
- Use texture views to expose different parts of textures
- Create new bind groups when resources change (it's lightweight)
- Use `queue.writeTexture()` for CPU→GPU data transfer
