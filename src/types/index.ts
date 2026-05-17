export interface Point {
  x: number
  y: number
}

export interface Selection {
  x: number
  y: number
  width: number
  height: number
}

export type ToolType = 
  | 'select'
  | 'rectangle'
  | 'ellipse'
  | 'arrow'
  | 'line'
  | 'pencil'
  | 'mosaic'
  | 'image'
  | 'text'

export interface Annotation {
  id: string
  type: ToolType
  points: Point[]
  color: HSLColor
  lineWidth: number
  text?: string
  fontSize?: number
  imageData?: string
  mosaicBlockSize?: number
}

export interface HSLColor {
  h: number
  s: number
  l: number
}

export interface MosaicOptions {
  blockSize: number
}
