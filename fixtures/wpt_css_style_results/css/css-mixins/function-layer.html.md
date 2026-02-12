# css/css-mixins/function-layer.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-layer.html"
}
```

## style[0]

```css

    @layer {
      @function --f() { result: 1px; }
    }
    #target {
      --actual: --f();
      --expected: 1px;
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

    @layer {
      @function --f() { result: 1px; }
    }
    @layer {
      @function --f() { result: 2px; }
    }
    #target {
      --actual: --f();
      --expected: 2px;
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

    @layer {
      @function --f() { result: 1px; }
    }
    @layer {
      @function --f() { result: 2px; }
    }
    @function --f() { result: 3px; }
    #target {
      --actual: --f();
      --expected: 3px;
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

    @function --f() { result: 3px; }
    @layer {
      @function --f() { result: 1px; }
    }
    @layer {
      @function --f() { result: 2px; }
    }
    #target {
      --actual: --f();
      --expected: 3px;
    }
  
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

    @layer base {
      @function --f() { result: 10px; }
    }
    #target {
      --actual: --f();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

    @layer base {
      @function --f() { result: 10px; }
    }
    @layer theme {
      @function --f() { result: 20px; }
    }
    #target {
      --actual: --f();
      --expected: 20px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css

    @layer theme, base;

    @layer base {
      @function --f() { result: 10px; } /* Winner */
    }
    @layer theme {
      @function --f() { result: 20px; }
    }
    #target {
      --actual: --f();
      --expected: 10px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
