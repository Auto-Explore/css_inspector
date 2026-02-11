# css/CSS2/css1/c71-fwd-parsing-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c71-fwd-parsing-003.xht"
}
```

## style[0]

```css
<![CDATA[
   body { color: navy; }
   p.eighteena {text-decoration: underline overline line-through diagonal;
               font: bold highlighted 100% sans-serif;}
   p.eighteenb {text-decoration: underline overline line-through diagonal;
               font: bold highlighted 100% serif;}
   em, p.nineteena ! em, strong {font-size: 200%; }
  ]]>
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
