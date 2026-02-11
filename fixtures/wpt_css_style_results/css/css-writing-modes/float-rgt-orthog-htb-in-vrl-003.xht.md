# css/css-writing-modes/float-rgt-orthog-htb-in-vrl-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-rgt-orthog-htb-in-vrl-003.xht"
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

  div#orthog-htb-float-right
    {
      background-color: blue;
      color: white;
      float: right;
      writing-mode: horizontal-tb;
    }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
