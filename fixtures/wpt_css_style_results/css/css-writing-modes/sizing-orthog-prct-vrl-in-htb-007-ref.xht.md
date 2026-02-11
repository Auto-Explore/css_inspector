# css/css-writing-modes/sizing-orthog-prct-vrl-in-htb-007-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-vrl-in-htb-007-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
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
      height: 200px;
      left: 8px;
      position: absolute;
      top: 52px;
    }

  p#sentence-after
    {
      padding-bottom: 122px;
      top: 258px; /* 52px + 3px + 200px + 3px == 258px */
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
