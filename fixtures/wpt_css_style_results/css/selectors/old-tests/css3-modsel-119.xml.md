# css/selectors/old-tests/css3-modsel-119.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-119.xml"
}
```

## style[0]

```css
<![CDATA[@namespace html url(http://www.w3.org/1999/xhtml);
@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
div.test *:not(*|div) { display : block ; margin-bottom : 1em ;
                            background-color : red }
div.test > *:not(*|p):not(*|div) { background-color : lime }
div.stub > *:not(*|div) { background-color : lime }
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
