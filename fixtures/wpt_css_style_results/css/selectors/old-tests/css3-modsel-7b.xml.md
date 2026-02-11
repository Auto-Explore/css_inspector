# css/selectors/old-tests/css3-modsel-7b.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/old-tests/css3-modsel-7b.xml"
}
```

## style[0]

```css
<![CDATA[
p { background: lime; }
[title~="hello world"] { background: red; }
/* Section 6.3.1: Represents the att attribute whose value is a
space-separated list of words, one of which is exactly "val". If this
selector is used, the words in the value must not contain spaces
(since they are separated by spaces). */
]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
