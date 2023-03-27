# wfc
A naive implementation/extrapolation of [Wave Function Collapse](https://github.com/mxgmn/WaveFunctionCollapse),
designed and implemented with minimal research and maximal incompetence.  

Due to my lack of expertise, I've inadvertently invented an intellectually
insufficient and likely inexplicably inconsistent terminology of my own. 
Thus, the table below may be of use if you're planning on examining the code:

## The Terrible Terminology

| name | used in | meaning |
| ---- | ------- | ------- |
| Soup | struct name | A **sup**erposition of states; Effectively an object storing the possible states of a cell. |
| certain | `Soup::certain`, `Grid::collapse_certain` | The certain state of a Soup; Only defined if there is exactly 1 possible state the Soup could be in; `None` otherwise.
| induce | `Rules::induce()` | `Induce` as in `induction`; The process of inferring rules based on the available information.
