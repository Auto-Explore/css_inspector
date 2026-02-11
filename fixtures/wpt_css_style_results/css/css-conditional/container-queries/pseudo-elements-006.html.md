# css/css-conditional/container-queries/pseudo-elements-006.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/pseudo-elements-006.html"
}
```

## style[0]

```css

  .container { container-type: inline-size; }
  #target { display: list-item; }
  @container (max-width: 200px) {
    #target::before { content: "PASS"; font-size: 10cqw; }
    #target::after { font-size: 10cqw; }
    #target::marker { font-size: 10cqw; }
    #target::first-line { font-size: 10cqw; }
    #target::first-letter { font-size: 10cqw; }
  }
  @container ((min-width: 300px) and (max-width: 350px)) {
    #outer::first-line { font-size: 10cqw; }
    #outer::first-letter { font-size: 10cqw; }
  }
  dialog::backdrop { font-size: 0px; }
  @container (max-width: 100px) {
    dialog::backdrop { font-size: 10cqw; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
