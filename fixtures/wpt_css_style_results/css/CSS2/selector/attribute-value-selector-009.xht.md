# css/CSS2/selector/attribute-value-selector-009.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/selector/attribute-value-selector-009.xht"
}
```

## style[0]

```css
<![CDATA[
      div[title |= "es"] { color:white;background-color:green; }
      p[title |= "es"] { color:white;background-color:green; }
      div[title |= "en-GB"] { color:white;background-color:green; }
      p[title |= "fr"] { color:white;background-color:green; }
      em[title |= "de"] { color:white;background-color:green; }
      div[lang=" no"] { color:white;background-color:green; }
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
