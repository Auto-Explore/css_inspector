# css/selectors/old-tests/css3-modsel-51.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-51.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
div.stub > p {color : red }
div.stub > a|* { color : red ; display : block ; margin-bottom : 1em }
div.stub *|*:not([test="1"]) { color : lime }]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
