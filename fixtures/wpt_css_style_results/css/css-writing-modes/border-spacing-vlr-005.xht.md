# css/css-writing-modes/border-spacing-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-spacing-vlr-005.xht"
}
```

## style[0]

```css
<![CDATA[
  table
    {
      border-spacing: 0em 0.5em; /* computes to logical vertical border-spacing: 10px */
      font: 20px/1 Ahem; /* computes to 20px/20px */
      padding: 0em 1.25em; /* computes to padding-left: 25px and padding-right: 25px */
      writing-mode: vertical-lr;
    }

  td
    {
      height: 1em;
      padding: 0em;
      width: 0.5em;
    }

  /*

  0px             25px     35px     45px     55px     65px     75px            100px
  | padding-left   |  left  |  2nd   | middle |  1st   |  right | padding-right|
  |   of table     |  vert. |  row   |  vert. |  right |  vert. |   of table   |
  |                | border |        | border |  most  | border |              |
  |                | spacing|        | spacing|  row   | spacing|              |
20|                |        |        |        |        |        |              |
px|                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
40|                |        |        |        |        |        |              |
px|                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
60|                |        |        |        |        |        |              |
px|                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
80|                |        |        |        |        |        |              |
px|                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
  |                |        |        |        |        |        |              |
00|                |        |        |        |        |        |              |
px|                |        |        |        |        |        |              |

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
