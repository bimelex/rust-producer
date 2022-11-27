pub mod agent_model {
    use serde::{Serialize, Deserialize};

    #[derive(Deserialize, Serialize)]
    pub struct AgentInfo{
        pub agentId: i32,
        pub agentName: String,
        pub connectYn: i32,
        pub groupId: i32,
    }

}

pub mod perf_model {
    use serde::{Serialize, Deserialize};

    #[derive(Deserialize, Serialize)]
    pub struct RealTime {
        pub agentId : i32,
        pub agentName : String,
        pub ontuneTime : u64 ,
        pub _user : i32,
        pub _sys : i32,
        pub _idle : i32,
        pub _processorCount : i32,
        pub _runQueue : i32,
        pub _blockQueue : i32,
        pub _waitQueue : i32,
        pub _pQueue : i32,
        pub _pcrateUser : i32,
        pub _pcrateSys : i32,
        pub _memorySize : i32,
        pub _memoryUsed : i32,
        pub _memoryPinned : i32,
        pub _memorySys : i32,
        pub _memoryUser : i32,
        pub _memoryCache : i32,
        pub _avm : i32,
        pub _pagingSpaceIn : i32,
        pub _pagingSpaceOut : i32,
        pub _fileSystemIn : i32,
        pub _fileSystemOut : i32,
        pub _memoryScan : i32,
        pub _memoryFreed : i32,
        pub _swapSize : i32,
        pub _swapUsed : i32,
        pub _swapActive : i32,
        pub _fork : i32,
        pub _exec : i32,
        pub _interupt : i32,
        pub _systemCall : i32,
        pub _contextSwitch : i32,
        pub _semaphore : i32,
        pub _msg : i32,
        pub _diskReadWrite : i32,
        pub _diskIops : i32,
        pub _networkReadWrite : i32,
        pub _networkIops : i32,
        pub _topCommandId : i32,
        pub _topCommandCount : i32,
        pub _topUserId : i32,
        pub _topCpu : i32,
        pub _topDiskId : i32,
        pub _topVgId : i32,
        pub _topBusy : i32,
        pub _maxPid : i32,
        pub _threadCount : i32,
        pub _pidCount : i32,
        pub _linuxBuffer : i32,
        pub _linuxCached : i32,
        pub _linuxSrec : i32,
        pub _memUsedMb : i32,
        pub _irq : i32,
        pub _softIrq : i32,
        pub _swapUsedMb : i32,
        pub _dusm : i32,
    }
}