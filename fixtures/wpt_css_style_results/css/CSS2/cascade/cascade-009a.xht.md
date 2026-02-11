# css/CSS2/cascade/cascade-009a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/cascade-009a.xht"
}
```

## style[0]

```css
<![CDATA[
  div {font-size: 100px;}

  div:first-letter {color: green !important;}

  div:first-letter {color: red;}
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
