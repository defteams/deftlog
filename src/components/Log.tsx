import ReactJson from 'react-json-view';

const Log = ({log, collapsed}: { log: any, collapsed: boolean}) => {
  return (
    <div className="log pb-2">
      <div className="text-sm">
        # {log.channel} | {log.level_name} | {log.datetime}
      </div>
      <ReactJson
        name={log.message}
        src={log.context}
        collapsed={collapsed}
        theme="ocean"
        iconStyle="circle"
      />
      <hr />
    </div>
  )
}
export default Log;