# css/css-writing-modes/float-shrink-to-fit-vrl-vlr-016.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-shrink-to-fit-vrl-vlr-016.xht"
}
```

## style[0]

```css
<![CDATA[
  div.test
    {
      background-color: red;
      font: 100px/1 Ahem; /* computes to 100px/100px */
    }

  div.floated-right
    {
      float: right;
    }

  div.floated-left
    {
      float: left;
    }

  div.vrl
    {
      writing-mode: vertical-rl;
    }

  div.vlr
    {
      writing-mode: vertical-lr;
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
