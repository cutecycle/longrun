# longrun

Naive Universal Durable Execution System.
## Goals
* If your language has lambdas, your language has LongRun.
* Extremely simple to implement.


# Specification
## StateStore 
StateStore interface holds program state, and program state is defined by "the set of every persistent variable." it should be compatible with any system that can store a key and a value, including but not limited to rdbms, kv stores, local filesystem, blob storage, memory, etc.

### Requirements

#### Preferred
* Eventual consistency 
#### Mandatory


## Variable 



```json
{
    
}
```


## Task 

Task is an object created by the StateStore that either references a newly created task or an existing task. it consists primarily of a name, TaskId, and the function to execute.


```json

{
    //immutable
    "Name": "",
    "TaskId": "",
    "StateId": "",
    "StartTime": "", 
    "Version": 0

    //Mutable only upon completion
    "EndTime": "",
    "Status": 200
    
}
```


## Language requirements

### Mandatory
* Lambdas or function pointers
* Interface or Class-level default implementations
### Preferred
* Result types
* 