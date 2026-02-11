# css/selectors/old-tests/css3-modsel-57.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-57.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
*|p, *|q, *|r { display : block ; margin-bottom : 1em }
*|p, *|r { background-color : red }
div.stub *:not([a|title]) {background-color : lime }
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
