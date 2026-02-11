# css/css-text/white-space/text-wrap-balance-before-after-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/text-wrap-balance-before-after-002.html"
}
```

## style[0]

```css

.container {
  font: 16px monospace;
  margin: 1em;
  width: 60ch;
}

.text {
  text-wrap: balance;
  outline: 1px dashed gray;
}

.text:before {
  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
  letter-spacing: -0.15ch;
  display: block;
  margin-bottom: 1em;
}

.text:after {
  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
  text-wrap-style: stable;
  display: block;
  margin-top: 1em;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
