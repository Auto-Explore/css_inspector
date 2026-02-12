# css/CSS2/css1/c71-fwd-parsing-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c71-fwd-parsing-001.xht"
}
```

## style[0]

```css
<![CDATA[
   body { color: navy; }
   p.sixa {border-width: medium; border-style: solid;}
   p.sixb {border-width: funny; border-style: solid;}
   p.sixc {border-width: 50zu; border-style: solid;}
   p.sixd {border-width: px; border-style: solid;}
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
