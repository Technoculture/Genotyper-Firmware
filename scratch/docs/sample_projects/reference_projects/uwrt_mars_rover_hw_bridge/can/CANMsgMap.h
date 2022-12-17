#pragma once

#include <iterator>
#include <unordered_map>

#include "uwrt_mars_rover_can_enums.h"

namespace HWBRIDGE {

class CANMsgMap {
 public:
  using CANSignalMapType = std::unordered_map<const CANSIGNAL, CANSignalValue_t>;
  using CANMsgMapType    = std::unordered_map<const CANID, CANSignalMapType>;

  CANMsgMap() = delete;

  // range based constructor
  CANMsgMap(CANMsgMapType::iterator first, CANMsgMapType::iterator last);

  // copy constructor and assignment
  CANMsgMap(const CANMsgMap &) = default;
  CANMsgMap &operator=(const CANMsgMap &) = default;

  // move constructor and assignment
  CANMsgMap(CANMsgMap &&) = default;
  CANMsgMap &operator=(CANMsgMap &&) = default;

  // initializer list constructor
  CANMsgMap(std::initializer_list<std::pair<const CANID, CANSignalMapType>> init);

  bool getSignalValue(CANID msgID, CANSIGNAL signalName, CANSignalValue_t &signalValue) const;
  bool setSignalValue(CANID msgID, CANSIGNAL signalName, CANSignalValue_t signalValue);
  bool contains(CANID msgID) const;
  bool contains(CANID msgID, CANSIGNAL signalName) const;

  CANSignalMapType &at(CANID msgID);
  CANMsgMapType::iterator begin();
  CANMsgMapType::iterator end();

  const CANSignalMapType &at(CANID msgID) const;
  const CANMsgMapType::const_iterator begin() const;
  const CANMsgMapType::const_iterator end() const;

 private:
  CANMsgMapType m_map;
};
}  // namespace HWBRIDGE
