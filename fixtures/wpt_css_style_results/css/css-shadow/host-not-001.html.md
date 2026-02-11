# css/css-shadow/host-not-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/host-not-001.html"
}
```

## style[0]

```css

      :host {
        width: 100px;
        height: 100px;
        background-color: green;
      }
      div:host {
        background-color: red;
      }
      :not(div):host {
        background-color: red;
      }
      :host:not(div) {
        background-color: red;
      }
      :host:is(div) {
        background-color: red;
      }
      :is(div):host {
        background-color: red;
      }
      :host:not(:hover) {
        background-color: red;
      }
      :host:not(:defined) {
        background-color: red;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
