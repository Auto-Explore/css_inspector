# css/css-writing-modes/sizing-orthog-vlr-in-htb-019-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vlr-in-htb-019-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
    }

  p
    {
      left: 8px;
      position: absolute;
      writing-mode: horizontal-tb;
    }

  p#sentence-before
    {
      margin-top: 8px;
    }

  div
    {
      border: blue solid 3px;
      height: 394px;
      left: 8px;
      position: absolute;
      top: 52px;
    }

  p#sentence-after
    {
      top: 452px; /* 52px + 400px == 452px */
      padding-bottom: 1em;
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
