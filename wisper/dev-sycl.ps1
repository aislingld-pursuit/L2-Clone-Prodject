# Advanced / experimental: Intel oneAPI SYCL (not a primary release target).
# For Intel iGPU on Windows, prefer .\dev.ps1 -GpuBackend vulkan instead.
. "$PSScriptRoot\dev.ps1" @args -GpuBackend sycl
