# css/css-grid/grid-items/grid-item-overflow-auto-max-height-percentage-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-items/grid-item-overflow-auto-max-height-percentage-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
}

.webconsole-app {
  display: grid;
}

.sidebar {
  display: flex;
  flex: 1;
  flex-direction: row;
  background: blue;
}

.controlled {
  display: flex;
  overflow: auto;
}

.sidebar-wrapper {
  display: grid;
  grid-template-columns: 100px;
  grid-template-rows: 50px 1fr;
  overflow: hidden;
}

.sidebar-contents {
  grid-row: 2 / 3;
  overflow: auto;
}

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
