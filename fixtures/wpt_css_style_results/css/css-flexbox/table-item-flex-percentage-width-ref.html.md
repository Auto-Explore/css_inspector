# css/css-flexbox/table-item-flex-percentage-width-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/table-item-flex-percentage-width-ref.html"
}
```

## style[0]

```css

  html,body {
    color:black; background-color:white; font:20px/1 Ahem; padding:0; margin:0;
  }

  .container {
    display: flex;
    width: 180px;
    border: 1px solid blue;
  }

  .container > * {
    background-color: cyan;
    display: block;
  }
  .test1 > * {
    flex-shrink: 0;
    width: 100%;
  }
  .test2 > * {
    flex-shrink: 0;
    width: 50%;
  }
  .test3 > * {
    flex-shrink: 0.5;
    width: 50%;
  }
  .test4 > * {
    flex-grow: 0.1;
    width: 50%;
  }
  .test5 > * {
    flex-shrink: 0.2;
    flex-grow: 0.5;
    width: 10%;
  }
  .base > * {
    flex-basis: 100px;
  }
  .large-base {
    width: 100px;
  }
  .large-base > * {
    flex-basis: 200px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
