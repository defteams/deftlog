import JsonView from "react18-json-view";

export default function LogDetail({ data }: any) {
  return (
    <JsonView
      className="overflow-x-auto"
      src={data?.context}
      collapsed={2}
      collapseStringMode="directly"
      collapseStringsAfterLength={50}
      displaySize={true}
      theme="atom"
    />
  );
}
