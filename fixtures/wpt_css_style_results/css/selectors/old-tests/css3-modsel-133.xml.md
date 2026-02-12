# css/selectors/old-tests/css3-modsel-133.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-133.xml"
}
```

## style[0]

```css
<![CDATA[@namespace a url(http://www.example.org/a);
@namespace b url(http://www.example.org/b);
@namespace html url(http://www.w3.org/1999/xhtml);
*|p, *|q, *|r, *|s { display : block ; margin-bottom : 1em }
*|p.foo, *|q, *|s { background-color : red }
div.stub html|*:not([*|lang|="en"]),
  div.stub *|*:not(html|*):not([a|foo|="un-d"]) { background-color : lime }
]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
