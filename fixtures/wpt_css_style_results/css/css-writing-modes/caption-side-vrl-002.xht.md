# css/css-writing-modes/caption-side-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/caption-side-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#reference-overlapped-red
    {
      background: url("support/pattern-gr-gr-100x100.png") no-repeat;
      height: 100px;
      position: absolute;
      width: 100px;
      z-index: -1;
    }

  table#test-overlapping-green
    {
      border-spacing: 0px;
      caption-side: top;
      font: 50px/1 Ahem; /* computes to 50px/50px */
      writing-mode: vertical-rl;
    }

  caption
    {
      color: green;
    }

  td
    {
      color: transparent;
      padding: 0px;
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
