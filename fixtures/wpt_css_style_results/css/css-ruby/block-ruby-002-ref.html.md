# css/css-ruby/block-ruby-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/block-ruby-002-ref.html"
}
```

## style[0]

```css

div,span { background: lightblue; }
ruby { display: ruby; }
rbb { display: ruby; background: lightblue; }
grid { display: grid; }
.mbp {
  margin: 1px 3px 5px 7px;
  padding: 3px 5px 7px 1px;
  border-width: 3px 1px 7px 5px;
  border-style: solid;
}
rbc {
  display: ruby-base-container;
  unicode-bidi: isolate;
}
rtc {
  display: ruby-text-container;
  unicode-bidi: isolate;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
