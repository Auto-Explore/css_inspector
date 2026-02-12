# css/selectors/old-tests/css3-modsel-124b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-124b.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
*|p, *|q, *|r, *|s { display : block ; margin-bottom : 1em }
*|p, *|r, *|s { background-color : lime ! important }
div.stub *:not([a|title="foo"]) {background-color : red }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
