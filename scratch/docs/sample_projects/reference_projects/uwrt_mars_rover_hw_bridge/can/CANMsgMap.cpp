#include "CANMsgMap.h"

using namespace HWBRIDGE;

CANMsgMap::CANMsgMap(CANMsgMapType::iterator first, CANMsgMapType::iterator last) : m_map(first, last) {}

CANMsgMap::CANMsgMap(std::initializer_list<std::pair<const CANID, CANSignalMapType>> init) : m_map(init) {}

bool CANMsgMap::getSignalValue(CANID msgID, CANSIGNAL signalName, CANSignalValue_t& signalValue) const {
  if (contains(msgID, signalName)) {
    signalValue = m_map.at(msgID).at(signalName);
    return true;
  }
  return false;
}

bool CANMsgMap::setSignalValue(CANID msgID, CANSIGNAL signalName, CANSignalValue_t signalValue) {
  if (contains(msgID, signalName)) {
    m_map[msgID][signalName] = signalValue;
    return true;
  }
  return false;
}

bool CANMsgMap::contains(CANID msgID) const {
  return m_map.contains(msgID);
}

bool CANMsgMap::contains(CANID msgID, CANSIGNAL signalName) const {
  return contains(msgID) && m_map.at(msgID).contains(signalName);
}

CANMsgMap::CANSignalMapType& CANMsgMap::at(CANID msgID) {
  return m_map.at(msgID);
}

CANMsgMap::CANMsgMapType::iterator CANMsgMap::begin() {
  return m_map.begin();
}

CANMsgMap::CANMsgMapType::iterator CANMsgMap::end() {
  return m_map.end();
}

const CANMsgMap::CANSignalMapType& CANMsgMap::at(CANID msgID) const {
  return m_map.at(msgID);
}

const CANMsgMap::CANMsgMapType::const_iterator CANMsgMap::begin() const {
  return m_map.begin();
}

const CANMsgMap::CANMsgMapType::const_iterator CANMsgMap::end() const {
  return m_map.end();
}
