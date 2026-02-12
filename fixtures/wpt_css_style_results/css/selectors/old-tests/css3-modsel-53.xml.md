# css/selectors/old-tests/css3-modsel-53.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-53.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
div.stub p { color : red }
div.stub > a|*, div.stub > b|* { color : red ; display : block ; margin-bottom : 1em }
div.stub *|*:not([test|="foo-bar"]) { color : lime }
div.stub *|p:not([lang|="en-us"]) { color : lime }
div.stub b|*[test|="foo2-bar"] { color : lime }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
