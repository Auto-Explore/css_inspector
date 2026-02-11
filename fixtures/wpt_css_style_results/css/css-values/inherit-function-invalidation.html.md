# css/css-values/inherit-function-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/inherit-function-invalidation.html"
}
```

## style[0]

```css

    #outer {
      --z: 2;
    }
    #target {
      z-index: inherit(--z);
    }
  
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

    #outer {
      --z: 2;
    }
    #target {
      z-index: inherit(--z, 7);
    }
  
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

    @property --z {
      syntax: "<integer>";
      inherits: false;
      initial-value: 0;
    }
    #outer {
      --z: 2;
    }
    #target {
      z-index: inherit(--z, 7);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
