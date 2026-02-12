# css/css-inline/text-box-trim/text-box-trim-ruby-end-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/text-box-trim/text-box-trim-ruby-end-001.html"
}
```

## style[0]

```css

.spacer {
  height: 40px;
  background: lightgray;
}
.target {
  font: 40px/1 Ahem;
  ruby-position: under;
  text-box-trim: trim-end;
  text-box-edge: text;
}
rt {
  /* The Ruby annotation positioning is UA-dependent. In order to ref-test the
     trimming of parts above the ascent, hide the Ruby annotation. */
  color: transparent;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
