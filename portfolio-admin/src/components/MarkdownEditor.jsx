import MDEditor from '@uiw/react-md-editor';
import '@uiw/react-md-editor/markdown-editor.css';

function parseHeight(v) {
  if (typeof v === 'number') return v;
  if (typeof v === 'string' && /^\d+/.test(v)) return parseInt(v, 10) || 320;
  return 320;
}

export default function MarkdownEditor({ value = '', onChange, placeholder, minHeight = 320 }) {
  const height = parseHeight(minHeight);
  return (
    <div data-color-mode="dark">
      <MDEditor
        value={value}
        onChange={onChange}
        placeholder={placeholder}
        height={height}
        visibleDragbar
        preview="live"
        style={{ background: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 6 }}
      />
    </div>
  );
}
