# css/selectors/old-tests/css3-modsel-93.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-93.xml"
}
```

## style[0]

```css
<![CDATA[@namespace test url(http://www.example.org/b);
*|testA { background-color : red }
|testA {background-color : lime }]]>
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
