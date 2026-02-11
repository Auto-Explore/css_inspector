# css/css-view-transitions/navigation/at-rule-in-layer-cascade-external-stylesheet.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/at-rule-in-layer-cascade-external-stylesheet.html"
}
```

## style[0]

```css

  @layer inertA, inertB, active;
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

  @import "resources/opt-out-style.css" layer(inertA);
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```

## style[2]

```css

  @import "resources/opt-in-style.css" layer(active);
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```

## style[3]

```css

  @import "resources/opt-out-style.css" layer(inertB);
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```
