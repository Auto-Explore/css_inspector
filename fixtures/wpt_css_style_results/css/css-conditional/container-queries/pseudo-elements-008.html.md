# css/css-conditional/container-queries/pseudo-elements-008.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-008.html"
}
```

## style[0]

```css

  .container { container-type: inline-size; }
  #target { display: list-item; }
  #target::before { content: "PASS"; font-size: 10cqw; }
  #target::after { font-size: 10cqw; }
  #target::marker { font-size: 10cqw; }
  #target::first-line { font-size: 10cqw; }
  #target::first-letter { font-size: 10cqw; }
  #target::highlight(foo) {
    text-decoration-line: underline;
    text-decoration-thickness: 10cqw;
  }
  #outer::first-line { font-size: 10cqw; }
  #outer::first-letter { font-size: 10cqw; }
  dialog::backdrop { font-size: 10cqw; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
