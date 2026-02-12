# css/css-writing-modes/float-rgt-orthog-htb-in-vlr-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-rgt-orthog-htb-in-vlr-003-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
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
      bottom: 16px;
      color: white;
      left: 16px;
      position: absolute;
      writing-mode: horizontal-tb;
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
