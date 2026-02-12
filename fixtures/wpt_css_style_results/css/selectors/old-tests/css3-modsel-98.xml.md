# css/selectors/old-tests/css3-modsel-98.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-98.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
*|p, *|q, *|r, *|s { display : block ; margin-bottom : 1em }
*|q, *|t { background-color : red }
*[a|title="foo"] {background-color : lime }
*[a|title=footwo] {background-color : lime }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
