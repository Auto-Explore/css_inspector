# css/CSS2/lists/counter-reset-increment-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/lists/counter-reset-increment-002.xht"
}
```

## style[0]

```css
<![CDATA[
  ol
  {
  counter-reset: list-item -4;
  list-style-type: none;
  }

  li:before
  {
  content: counter(list-item) ". ";
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
