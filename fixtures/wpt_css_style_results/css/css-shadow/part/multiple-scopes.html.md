# css/css-shadow/part/multiple-scopes.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/multiple-scopes.html"
}
```

## style[0]

```css


::part(e1)::file-selector-button { border-top-width: 7px; }
::part(p1)::file-selector-button { border-top-color: red; }
::file-selector-button { border-top-style: dashed; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

      ::part(e1)::file-selector-button { border-right-width: 6px; }
      ::part(p1)::file-selector-button { border-right-color: lime; }
      ::file-selector-button { border-right-style: dotted; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

      ::part(e1)::file-selector-button { border-bottom-width: 5px; }
      ::part(p1)::file-selector-button { border-bottom-color: red; }
      ::file-selector-button { border: 3px double black; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
