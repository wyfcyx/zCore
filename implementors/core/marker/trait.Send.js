(function() {var implementors = {};
implementors["kernel_hal"] = [{"text":"impl Send for VectorRegs","synthetic":true,"types":[]},{"text":"impl Send for U128","synthetic":true,"types":[]},{"text":"impl Send for Thread","synthetic":true,"types":[]},{"text":"impl Send for PageTable","synthetic":true,"types":[]},{"text":"impl Send for PhysFrame","synthetic":true,"types":[]},{"text":"impl Send for InterruptManager","synthetic":true,"types":[]},{"text":"impl Send for SleepFuture","synthetic":true,"types":[]},{"text":"impl Send for SerialFuture","synthetic":true,"types":[]},{"text":"impl Send for MMUFlags","synthetic":true,"types":[]},{"text":"impl Send for CachePolicy","synthetic":true,"types":[]},{"text":"impl&lt;P&gt; Send for IoVec&lt;P&gt;","synthetic":true,"types":[]},{"text":"impl&lt;P&gt; Send for IoVecs&lt;P&gt;","synthetic":true,"types":[]},{"text":"impl Send for In","synthetic":true,"types":[]},{"text":"impl Send for Out","synthetic":true,"types":[]},{"text":"impl Send for InOut","synthetic":true,"types":[]},{"text":"impl Send for Error","synthetic":true,"types":[]},{"text":"impl Send for VdsoConstants","synthetic":true,"types":[]},{"text":"impl Send for Features","synthetic":true,"types":[]},{"text":"impl Send for VersionString","synthetic":true,"types":[]},{"text":"impl&lt;T, P:&nbsp;Policy&gt; Send for UserPtr&lt;T, P&gt;","synthetic":false,"types":[]}];
implementors["kernel_hal_unix"] = [{"text":"impl Send for Thread","synthetic":true,"types":[]},{"text":"impl Send for PageTable","synthetic":true,"types":[]},{"text":"impl Send for PhysFrame","synthetic":true,"types":[]}];
implementors["linux_object"] = [{"text":"impl Send for LxError","synthetic":true,"types":[]},{"text":"impl Send for MemBuf","synthetic":true,"types":[]},{"text":"impl Send for FcntlFlags","synthetic":true,"types":[]},{"text":"impl Send for FileFlags","synthetic":true,"types":[]},{"text":"impl Send for File","synthetic":true,"types":[]},{"text":"impl Send for OpenOptions","synthetic":true,"types":[]},{"text":"impl Send for PipeData","synthetic":true,"types":[]},{"text":"impl Send for Pipe","synthetic":true,"types":[]},{"text":"impl Send for Pseudo","synthetic":true,"types":[]},{"text":"impl Send for RandomINodeData","synthetic":true,"types":[]},{"text":"impl Send for RandomINode","synthetic":true,"types":[]},{"text":"impl Send for STDIN","synthetic":true,"types":[]},{"text":"impl Send for STDOUT","synthetic":true,"types":[]},{"text":"impl Send for Stdin","synthetic":true,"types":[]},{"text":"impl Send for Stdout","synthetic":true,"types":[]},{"text":"impl Send for FileDesc","synthetic":true,"types":[]},{"text":"impl Send for SeekFrom","synthetic":true,"types":[]},{"text":"impl Send for PipeEnd","synthetic":true,"types":[]},{"text":"impl Send for SemidDs","synthetic":true,"types":[]},{"text":"impl Send for SemArray","synthetic":true,"types":[]},{"text":"impl Send for ShmidDs","synthetic":true,"types":[]},{"text":"impl Send for ShmIdentifier","synthetic":true,"types":[]},{"text":"impl Send for ShmGuard","synthetic":true,"types":[]},{"text":"impl Send for SemProc","synthetic":true,"types":[]},{"text":"impl Send for ShmProc","synthetic":true,"types":[]},{"text":"impl Send for IpcPerm","synthetic":true,"types":[]},{"text":"impl Send for LinuxElfLoader","synthetic":true,"types":[]},{"text":"impl Send for LinuxProcess","synthetic":true,"types":[]},{"text":"impl Send for RLimit","synthetic":true,"types":[]},{"text":"impl Send for Sigset","synthetic":true,"types":[]},{"text":"impl Send for SignalAction","synthetic":true,"types":[]},{"text":"impl Send for SigInfo","synthetic":true,"types":[]},{"text":"impl Send for SignalActionFlags","synthetic":true,"types":[]},{"text":"impl Send for MachineContext","synthetic":true,"types":[]},{"text":"impl Send for SignalUserContext","synthetic":true,"types":[]},{"text":"impl Send for SignalFrame","synthetic":true,"types":[]},{"text":"impl Send for SignalStackFlags","synthetic":true,"types":[]},{"text":"impl Send for SignalStack","synthetic":true,"types":[]},{"text":"impl Send for SiginfoFields","synthetic":true,"types":[]},{"text":"impl Send for SignalCode","synthetic":true,"types":[]},{"text":"impl Send for Signal","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl Send for EventBus","synthetic":true,"types":[]},{"text":"impl Send for Semaphore","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Send for SemaphoreGuard&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Send for LinuxThread","synthetic":true,"types":[]},{"text":"impl Send for TimeSpec","synthetic":true,"types":[]},{"text":"impl Send for TimeVal","synthetic":true,"types":[]},{"text":"impl Send for RUsage","synthetic":true,"types":[]},{"text":"impl Send for Tms","synthetic":true,"types":[]}];
implementors["linux_syscall"] = [{"text":"impl&lt;'a&gt; Send for Syscall&lt;'a&gt;","synthetic":true,"types":[]}];
implementors["zircon_loader"] = [{"text":"impl&lt;T&gt; Send for Images&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["zircon_object"] = [{"text":"impl Send for ZxError","synthetic":true,"types":[]},{"text":"impl Send for DebugLog","synthetic":true,"types":[]},{"text":"impl Send for Severity","synthetic":true,"types":[]},{"text":"impl Send for BusTransactionInitiator","synthetic":true,"types":[]},{"text":"impl Send for BtiInfo","synthetic":true,"types":[]},{"text":"impl Send for Interrupt","synthetic":true,"types":[]},{"text":"impl Send for InterruptFlags","synthetic":true,"types":[]},{"text":"impl Send for InterruptOptions","synthetic":true,"types":[]},{"text":"impl Send for Iommu","synthetic":true,"types":[]},{"text":"impl Send for IommuPerms","synthetic":true,"types":[]},{"text":"impl Send for MmioPcieAddressProvider","synthetic":true,"types":[]},{"text":"impl Send for PCIeBusDriver","synthetic":true,"types":[]},{"text":"impl Send for PcieDeviceInfo","synthetic":true,"types":[]},{"text":"impl Send for PcieDeviceKObject","synthetic":true,"types":[]},{"text":"impl Send for PioPcieAddressProvider","synthetic":true,"types":[]},{"text":"impl Send for PciEcamRegion","synthetic":true,"types":[]},{"text":"impl Send for MappedEcamRegion","synthetic":true,"types":[]},{"text":"impl Send for PinnedMemoryToken","synthetic":true,"types":[]},{"text":"impl Send for ResourceFlags","synthetic":true,"types":[]},{"text":"impl Send for Resource","synthetic":true,"types":[]},{"text":"impl Send for ResourceInfo","synthetic":true,"types":[]},{"text":"impl Send for PcieIrqMode","synthetic":true,"types":[]},{"text":"impl Send for PciAddrSpace","synthetic":true,"types":[]},{"text":"impl Send for ResourceKind","synthetic":true,"types":[]},{"text":"impl Send for PciIrqSwizzleLut","synthetic":true,"types":[]},{"text":"impl Send for PciInitArgsIrqs","synthetic":true,"types":[]},{"text":"impl Send for PciInitArgsHeader","synthetic":true,"types":[]},{"text":"impl Send for PciInitArgsAddrWindows","synthetic":true,"types":[]},{"text":"impl Send for Guest","synthetic":true,"types":[]},{"text":"impl Send for Vcpu","synthetic":true,"types":[]},{"text":"impl Send for VmmPageTable","synthetic":true,"types":[]},{"text":"impl Send for Channel","synthetic":true,"types":[]},{"text":"impl Send for MessagePacket","synthetic":true,"types":[]},{"text":"impl Send for Fifo","synthetic":true,"types":[]},{"text":"impl Send for Socket","synthetic":true,"types":[]},{"text":"impl Send for SocketFlags","synthetic":true,"types":[]},{"text":"impl Send for SocketInfo","synthetic":true,"types":[]},{"text":"impl Send for Handle","synthetic":true,"types":[]},{"text":"impl Send for HandleBasicInfo","synthetic":true,"types":[]},{"text":"impl Send for HandleInfo","synthetic":true,"types":[]},{"text":"impl Send for Rights","synthetic":true,"types":[]},{"text":"impl Send for Signal","synthetic":true,"types":[]},{"text":"impl Send for KObjectBase","synthetic":true,"types":[]},{"text":"impl Send for DummyObject","synthetic":true,"types":[]},{"text":"impl Send for Event","synthetic":true,"types":[]},{"text":"impl Send for EventPair","synthetic":true,"types":[]},{"text":"impl Send for Futex","synthetic":true,"types":[]},{"text":"impl Send for PortPacket","synthetic":true,"types":[]},{"text":"impl Send for PacketSignal","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestBell","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestMem","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestIo","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestVcpuInterrupt","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestVcpuStartup","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestVcpu","synthetic":true,"types":[]},{"text":"impl Send for PacketInterrupt","synthetic":true,"types":[]},{"text":"impl Send for PortPacketRepr","synthetic":true,"types":[]},{"text":"impl Send for Port","synthetic":true,"types":[]},{"text":"impl Send for PortOptions","synthetic":true,"types":[]},{"text":"impl Send for Timer","synthetic":true,"types":[]},{"text":"impl Send for Payload","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestVcpuData","synthetic":true,"types":[]},{"text":"impl Send for PacketType","synthetic":true,"types":[]},{"text":"impl Send for PacketGuestVcpuType","synthetic":true,"types":[]},{"text":"impl Send for PayloadRepr","synthetic":true,"types":[]},{"text":"impl Send for Slack","synthetic":true,"types":[]},{"text":"impl Send for Exceptionate","synthetic":true,"types":[]},{"text":"impl Send for ExceptionReport","synthetic":true,"types":[]},{"text":"impl Send for ExceptionObject","synthetic":true,"types":[]},{"text":"impl Send for Job","synthetic":true,"types":[]},{"text":"impl Send for JobInfo","synthetic":true,"types":[]},{"text":"impl Send for JobPolicy","synthetic":true,"types":[]},{"text":"impl Send for BasicPolicy","synthetic":true,"types":[]},{"text":"impl Send for TimerSlackPolicy","synthetic":true,"types":[]},{"text":"impl Send for Process","synthetic":true,"types":[]},{"text":"impl Send for ProcessInfo","synthetic":true,"types":[]},{"text":"impl Send for SuspendToken","synthetic":true,"types":[]},{"text":"impl Send for Thread","synthetic":true,"types":[]},{"text":"impl Send for ThreadFlag","synthetic":true,"types":[]},{"text":"impl Send for CurrentThread","synthetic":true,"types":[]},{"text":"impl Send for ThreadInfo","synthetic":true,"types":[]},{"text":"impl Send for ExceptionType","synthetic":true,"types":[]},{"text":"impl Send for SetPolicyOptions","synthetic":true,"types":[]},{"text":"impl Send for PolicyCondition","synthetic":true,"types":[]},{"text":"impl Send for PolicyAction","synthetic":true,"types":[]},{"text":"impl Send for Status","synthetic":true,"types":[]},{"text":"impl Send for ThreadStateKind","synthetic":true,"types":[]},{"text":"impl Send for ThreadState","synthetic":true,"types":[]},{"text":"impl Send for KCounter","synthetic":true,"types":[]},{"text":"impl Send for KCounterDescriptor","synthetic":true,"types":[]},{"text":"impl Send for KCounterDescriptorArray","synthetic":true,"types":[]},{"text":"impl Send for Stream","synthetic":true,"types":[]},{"text":"impl Send for StreamInfo","synthetic":true,"types":[]},{"text":"impl Send for VmarFlags","synthetic":true,"types":[]},{"text":"impl Send for VmAddressRegion","synthetic":true,"types":[]},{"text":"impl Send for VmarInfo","synthetic":true,"types":[]},{"text":"impl Send for VmMapping","synthetic":true,"types":[]},{"text":"impl Send for TaskStatsInfo","synthetic":true,"types":[]},{"text":"impl Send for VmObject","synthetic":true,"types":[]},{"text":"impl Send for VmoInfo","synthetic":true,"types":[]},{"text":"impl Send for VmoInfoFlags","synthetic":true,"types":[]},{"text":"impl Send for KERNEL_ASPACE","synthetic":true,"types":[]},{"text":"impl Send for SeekOrigin","synthetic":true,"types":[]}];
implementors["zircon_syscall"] = [{"text":"impl&lt;'a&gt; Send for Syscall&lt;'a&gt;","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()