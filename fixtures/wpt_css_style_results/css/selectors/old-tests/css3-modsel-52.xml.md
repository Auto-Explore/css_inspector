# css/selectors/old-tests/css3-modsel-52.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-52.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
div.stub p { color : red }
div.stub > a|*, div.stub > b|* { color : red ; display : block ; margin-bottom : 1em }
div.stub *|*:not([test~="foo"]) { color : lime }
div.stub *|p:not([class~="foo"]) { color : lime }
div.stub b|*[test~="foo2"] { color : lime }
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
