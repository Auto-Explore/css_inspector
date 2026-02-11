# css/css-values/inherit-function-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/inherit-function-basic.html"
}
```

## style[0]

```css

    #parent {
      --z: 2;
    }
    #target {
      --z: 13;
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

    :root {
      --z: 2;
    }
    #foo {
      --z: 13;
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

## style[2]

```css

    #foo {
      --z: 13;
      z-index: inherit(--z, 4);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

    #e1 { --v: e1; }
    #e2 { --v: e2 inherit(--v); }
    #e3 { --v: e3 inherit(--v); }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

    @property --f {
      syntax: "<length>";
      inherits: false;
      initial-value: 0px;
    }
    #e1 { font-size: 3px; --f: 1em; }
    #e2 { font-size: 5px; --f: calc(1em + inherit(--f)); }
    #e3 { font-size: 7px; --f: calc(1em + inherit(--f)); }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

    #parent {
      --z: 2;
    }
    #target {
      --z: 13;
      z-index: if(style(--z > inherit(--z)):4; else:7);
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “else”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
