# css/css-ruby/block-ruby-005.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/block-ruby-005.html"
}
```

## style[0]

```css

body {
  font:10px/1 monospace;
}

rbb, ruby { background:lightblue; overflow:hidden; width:3em; }
ruby { display: ruby; }
rbb { display: block ruby; }
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
  font-size: 200%;
}
rtc {
  display: ruby-text-container;
  unicode-bidi: isolate;
  ruby-position: under;
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
      "message": "Invalid value for property “display”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
