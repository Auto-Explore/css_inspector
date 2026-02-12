# css/css-view-transitions/scoped/crashtests/implicit-stacking-context.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/crashtests/implicit-stacking-context.html"
}
```

## style[0]

```css

  main {
    display: flex;
    flex-direction: row;
  }
  article {
    flex: 1;
    max-width: 200px;
    margin: 30px;
    font-size: 30px;
    background: green;
    border: 2px solid black;
  }
  section {
    flex: 1;
    overflow: auto;
    border-radius: 200px 20%;
    corner-shape: squircle;
    max-height: 200px;
    background: grey;
    padding: 10px;
  }
  ul {
    contain: view-transition;
  }
  li {
    padding: 10px;
    background: yellow;
    color: black;
    border: 1px solid purple;
    list-style: none;
  }
  body {
    display: flex;
    flex-direction: column;
  }
  header {
    display: flex;
    justify-content: flex-start;
  }
  main > *, li {
    view-transition-name: match-element;
  }

  ::view-transition-group(*) {
    animation-duration: 150ms;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
