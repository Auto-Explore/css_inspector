# css/css-writing-modes/float-lft-orthog-htb-in-vrl-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-lft-orthog-htb-in-vrl-002-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  div#vertical-parent
    {
      border: orange solid 8px;
      font-size: 32px;
      line-height: 1.25; /* computes to 40px */
    }

  div#orthog-horiz
    {
      background-color: blue;
      color: white;
      right: 16px;
      position: absolute;
      writing-mode: horizontal-tb;
    }

  span
    {
      position: relative;
      top: 40px;
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
