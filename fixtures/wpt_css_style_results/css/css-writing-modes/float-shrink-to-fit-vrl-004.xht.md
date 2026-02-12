# css/css-writing-modes/float-shrink-to-fit-vrl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-shrink-to-fit-vrl-004.xht"
}
```

## style[0]

```css
<![CDATA[
  div.floated-left
    {
      background-color: red;
      float: left;
      font: 100px/1 Ahem; /* computes to 100px/100px */
      writing-mode: vertical-rl;
    }

  div.left-border
    {
      border-left: red solid 1em;
    }

  div.right-border
    {
      border-right: red solid 1em;
    }

  div#reference-overlapped-green
    {
      background-color: green;
      height: 100px;
      position: relative;
      width: 100px;
      z-index: -1;
    }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
