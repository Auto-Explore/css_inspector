# css/selectors/old-tests/css3-modsel-120.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-120.xml"
}
```

## style[0]

```css
<![CDATA[@namespace html url(http://www.w3.org/1999/xhtml);
@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
div.stub > * { display : block ; margin-bottom : 1em ;
                            background-color : red }
div.stub > *:not(|p) { background-color : lime }
div.stub > *|l > *:not(|p) { background-color : red }]]>
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
