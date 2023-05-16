# SMAC

A naive implementation/extrapolation of [Wave Function Collapse](https://github.com/mxgmn/WaveFunctionCollapse),
designed and implemented with minimal research and maximal incompetence.  

## The Terrible Terminology Table

Due to my lack of expertise, I've inadvertently invented an intellectually
insufficient and likely inexplicably inconsistent terminology of my own. 
Thus, the table below may be of use if you're planning on examining the code:

[//]: # (sorry if you're reading this in plaintext, the table doesn't seem to
          render properly if I don't keep the entries in single lines)

| name | used in | meaning |
| ---- | ------- | ------- |
| Soup | struct name | A **sup**erposition of states; Effectively an object storing the possible states of a cell. |
| certain | `Soup::certain()`, `Grid::collapse_certain()`, etc. | The certain state of a Soup; Only defined if there is exactly 1 possible state the Soup could be in; `None` otherwise. |
| induce | `Rules::induce()` | `Induce` as in `induction`; The process of inferring rules based on the available information. Currently limited to noticing patterns between a Grid cell and its 8 neighbours. |

## Clarifications
### Demo
As the only currently implemented method of collapse is just randomly evaluating
successive permutations by sheer brute force, it's not uncommon[^1] for the demo
to get stuck in a particularly unfortunate path, which it's unlikely to ever finish.
Depending on your hardware, if you notice it getting stuck for more than a few seconds,
you may want to terminate the process and try again(or pick a different seed, if you're
using a deterministic `order` function).

[^1]: &leq; 6% of the time, by my extremely loose estimate

### Legal
I don't claim to be in any way responsible for the original concept or implementation of WFC.
This repository is entirely a passion project inspired by the original algorithm.

Also, I know I haven't added a licence to this project as of yet.
Think of it as a built-in safeguard &mdash; if you're considering using this code commercially,
legal trouble is only one of the *many* factors you should *really* reconsider.

### Hubris
I am quite aware that the current implementation isn't nearly as clean or performant as it could be,
nor is it abstracted to the extent I'd like. I don't make my to-do lists public, partly because
it'd expose the extent of my attention issues to the anonymous conglomerate, and partly because
the way they're worded is likely incomprehensible to anyone but me anyway.  
Suffice to say, I *do* have a list for this project, and it is (and will likely remain) **far from complete**.

### Initialism
It stands for **S**oup **MA**p **C**ollapse.
