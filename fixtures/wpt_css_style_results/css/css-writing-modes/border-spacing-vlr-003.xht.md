# css/css-writing-modes/border-spacing-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-spacing-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-spacing: 0.5em 0em; /* computes to logical horizontal border-spacing: 10px */
      font: 20px/1 Ahem; /* computes to 20px/20px */
      padding: 1.25em 0em; /* computes to padding-top: 25px and padding-bottom: 25px */
      writing-mode: vertical-lr;
    }

  td
    {
      height: 0.5em;
      padding: 0em;
      width: 1em;
    }

  /*

         0px       width of nth row        20px                           TOP
         |                                 |
         ===================================     0x (0em)                 |||
         | table padding-top 25px (1.25em) |                              |||
         |    with red background=color    |                            \ ||| /
         |    with red background=color    |                             \   /
         |    with red background=color    |                              \ /
         |    with red background=color    |                               v
         ===================================    25px (1.25em)
         |logical left border-spacing: 10px|
         |    with red background=color    |
         ===================================    35px (1.75em)
         |  height of 1st td 10px (0.5em)  |                              |||
         |    with red background=color    |                              |||
         ===================================    45px (2.25em)           \ ||| /
         |inter-column spacing 10px (0.5em)|                             \   /
         |    with red background=color    |                              \ /
         ===================================    55px (2.75em)              v
         |  height of 2nd td 10px (0.5em)  |
         |    with red background=color    |
         ===================================    65px (3.25em)
         |logical righ border-spacing: 10px|                              |||
         |    with red background=color    |                              |||
         ===================================    75px (3.75em)           \ ||| /
         |tble padding-bottom 25px (1.25em)|                             \   /
         |    with red background=color    |                              \ /
         |    with red background=color    |                               v
         |    with red background=color    |
         |    with red background=color    |
         ===================================   100px (5em)              BOTTOM

  */

  div#reference-overlapping-green
    {
      background-color: green;
      height: 6.25em;
      position: absolute;
      width: 6.25em;
    }

  div#reference-overlapped-red
    {
      background-color: red;
      height: 6.25em;
      position: absolute;
      width: 6.25em;
      z-index: -1;
    }

  table#test-overlapped-red
    {
      background-color: red;
    }

  table#test-overlapping-green
    {
      background-color: green;
    }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
