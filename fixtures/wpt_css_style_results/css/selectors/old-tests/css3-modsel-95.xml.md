# css/selectors/old-tests/css3-modsel-95.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-95.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
div.test * { background-color : red ; display : block ; margin-bottom : 1em }
div.test *|* { background-color : lime }]]>
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
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    },
    {
      "message": "Stray declaration outside a rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
