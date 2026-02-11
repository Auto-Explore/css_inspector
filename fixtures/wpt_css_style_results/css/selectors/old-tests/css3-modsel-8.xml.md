# css/selectors/old-tests/css3-modsel-8.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-8.xml"
}
```

## style[0]

```css
<![CDATA[p { background-color : red }
p[lang|="en"] { background-color : lime }
address { background-color : red }
address[lang="fi"] { background-color : lime }
span[lang|="fr"] { background-color : red }]]>
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
