1. ktx create --format R8G8B8A8_SRGB --encode uastc --assign-tf sRGB --generate-mipmap texture_atlas_1.png temp.ktx2
2. ktx transcode --target bc7 temp.ktx2 texture_atlas_1.ktx2
